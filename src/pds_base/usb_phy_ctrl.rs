#[doc = "Register `usb_phy_ctrl` reader"]
pub struct R(crate::R<USB_PHY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_PHY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_PHY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_PHY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_phy_ctrl` writer"]
pub struct W(crate::W<USB_PHY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_PHY_CTRL_SPEC>;
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
impl From<crate::W<USB_PHY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_PHY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_usb_phy_ponrst` reader - "]
pub type REG_USB_PHY_PONRST_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_phy_ponrst` writer - "]
pub type REG_USB_PHY_PONRST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_PHY_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_usb_phy_oscouten` reader - "]
pub type REG_USB_PHY_OSCOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_phy_oscouten` writer - "]
pub type REG_USB_PHY_OSCOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_PHY_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_usb_phy_xtlsel` reader - "]
pub type REG_USB_PHY_XTLSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_usb_phy_xtlsel` writer - "]
pub type REG_USB_PHY_XTLSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_PHY_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_usb_phy_outclksel` reader - "]
pub type REG_USB_PHY_OUTCLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_phy_outclksel` writer - "]
pub type REG_USB_PHY_OUTCLKSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_PHY_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_usb_phy_pllaliv` reader - "]
pub type REG_USB_PHY_PLLALIV_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_phy_pllaliv` writer - "]
pub type REG_USB_PHY_PLLALIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_PHY_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_pu_usb20_psw` reader - "]
pub type REG_PU_USB20_PSW_R = crate::BitReader<bool>;
#[doc = "Field `reg_pu_usb20_psw` writer - "]
pub type REG_PU_USB20_PSW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_PHY_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_7_31` reader - "]
pub type RESERVED_7_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_usb_phy_ponrst(&self) -> REG_USB_PHY_PONRST_R {
        REG_USB_PHY_PONRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_usb_phy_oscouten(&self) -> REG_USB_PHY_OSCOUTEN_R {
        REG_USB_PHY_OSCOUTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_usb_phy_xtlsel(&self) -> REG_USB_PHY_XTLSEL_R {
        REG_USB_PHY_XTLSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_usb_phy_outclksel(&self) -> REG_USB_PHY_OUTCLKSEL_R {
        REG_USB_PHY_OUTCLKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_usb_phy_pllaliv(&self) -> REG_USB_PHY_PLLALIV_R {
        REG_USB_PHY_PLLALIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_pu_usb20_psw(&self) -> REG_PU_USB20_PSW_R {
        REG_PU_USB20_PSW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn reserved_7_31(&self) -> RESERVED_7_31_R {
        RESERVED_7_31_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_ponrst(&mut self) -> REG_USB_PHY_PONRST_W<0> {
        REG_USB_PHY_PONRST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_oscouten(&mut self) -> REG_USB_PHY_OSCOUTEN_W<1> {
        REG_USB_PHY_OSCOUTEN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_xtlsel(&mut self) -> REG_USB_PHY_XTLSEL_W<2> {
        REG_USB_PHY_XTLSEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_outclksel(&mut self) -> REG_USB_PHY_OUTCLKSEL_W<4> {
        REG_USB_PHY_OUTCLKSEL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usb_phy_pllaliv(&mut self) -> REG_USB_PHY_PLLALIV_W<5> {
        REG_USB_PHY_PLLALIV_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pu_usb20_psw(&mut self) -> REG_PU_USB20_PSW_W<6> {
        REG_PU_USB20_PSW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_phy_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_phy_ctrl](index.html) module"]
pub struct USB_PHY_CTRL_SPEC;
impl crate::RegisterSpec for USB_PHY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_phy_ctrl::R](R) reader structure"]
impl crate::Readable for USB_PHY_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_phy_ctrl::W](W) writer structure"]
impl crate::Writable for USB_PHY_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_phy_ctrl to value 0"]
impl crate::Resettable for USB_PHY_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
