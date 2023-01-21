#[doc = "Register `usb_ctl` reader"]
pub struct R(crate::R<USB_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_ctl` writer"]
pub struct W(crate::W<USB_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USB_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_usb_sw_rst_n` reader - "]
pub type REG_USB_SW_RST_N_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_sw_rst_n` writer - "]
pub type REG_USB_SW_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTL_SPEC, bool, O>;
#[doc = "Field `reg_usb_ext_susp_n` reader - "]
pub type REG_USB_EXT_SUSP_N_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_ext_susp_n` writer - "]
pub type REG_USB_EXT_SUSP_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTL_SPEC, bool, O>;
#[doc = "Field `reg_usb_wakeup` reader - "]
pub type REG_USB_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_wakeup` writer - "]
pub type REG_USB_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTL_SPEC, bool, O>;
#[doc = "Field `reg_usb_l1_wakeup` reader - "]
pub type REG_USB_L1_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_l1_wakeup` writer - "]
pub type REG_USB_L1_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTL_SPEC, bool, O>;
#[doc = "Field `reg_usb_drvbus_pol` reader - "]
pub type REG_USB_DRVBUS_POL_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_drvbus_pol` writer - "]
pub type REG_USB_DRVBUS_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTL_SPEC, bool, O>;
#[doc = "Field `reg_usb_iddig` reader - "]
pub type REG_USB_IDDIG_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_iddig` writer - "]
pub type REG_USB_IDDIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTL_SPEC, bool, O>;
#[doc = "Field `reserved_6_31` reader - "]
pub type RESERVED_6_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_sw_rst_n(&self) -> REG_USB_SW_RST_N_R {
        REG_USB_SW_RST_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_ext_susp_n(&self) -> REG_USB_EXT_SUSP_N_R {
        REG_USB_EXT_SUSP_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_usb_wakeup(&self) -> REG_USB_WAKEUP_R {
        REG_USB_WAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_usb_l1_wakeup(&self) -> REG_USB_L1_WAKEUP_R {
        REG_USB_L1_WAKEUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_drvbus_pol(&self) -> REG_USB_DRVBUS_POL_R {
        REG_USB_DRVBUS_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_iddig(&self) -> REG_USB_IDDIG_R {
        REG_USB_IDDIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved_6_31(&self) -> RESERVED_6_31_R {
        RESERVED_6_31_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_sw_rst_n(&mut self) -> REG_USB_SW_RST_N_W<0> {
        REG_USB_SW_RST_N_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_ext_susp_n(&mut self) -> REG_USB_EXT_SUSP_N_W<1> {
        REG_USB_EXT_SUSP_N_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_wakeup(&mut self) -> REG_USB_WAKEUP_W<2> {
        REG_USB_WAKEUP_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_l1_wakeup(&mut self) -> REG_USB_L1_WAKEUP_W<3> {
        REG_USB_L1_WAKEUP_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_drvbus_pol(&mut self) -> REG_USB_DRVBUS_POL_W<4> {
        REG_USB_DRVBUS_POL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_iddig(&mut self) -> REG_USB_IDDIG_W<5> {
        REG_USB_IDDIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_ctl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ctl](index.html) module"]
pub struct USB_CTL_SPEC;
impl crate::RegisterSpec for USB_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_ctl::R](R) reader structure"]
impl crate::Readable for USB_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_ctl::W](W) writer structure"]
impl crate::Writable for USB_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_ctl to value 0x21"]
impl crate::Resettable for USB_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
