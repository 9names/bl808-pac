#[doc = "Register `sd_timeout_ctrl_sw_reset` reader"]
pub struct R(crate::R<SD_TIMEOUT_CTRL_SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_TIMEOUT_CTRL_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_TIMEOUT_CTRL_SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_TIMEOUT_CTRL_SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_timeout_ctrl_sw_reset` writer"]
pub struct W(crate::W<SD_TIMEOUT_CTRL_SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_TIMEOUT_CTRL_SW_RESET_SPEC>;
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
impl From<crate::W<SD_TIMEOUT_CTRL_SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_TIMEOUT_CTRL_SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timeout_value` reader - "]
pub type TIMEOUT_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timeout_value` writer - "]
pub type TIMEOUT_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_TIMEOUT_CTRL_SW_RESET_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_7_4` reader - "]
pub type RESERVED_7_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_rst_all` reader - "]
pub type SW_RST_ALL_R = crate::BitReader<bool>;
#[doc = "Field `sw_rst_all` writer - "]
pub type SW_RST_ALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_TIMEOUT_CTRL_SW_RESET_SPEC, bool, O>;
#[doc = "Field `sw_rst_cmd` reader - "]
pub type SW_RST_CMD_R = crate::BitReader<bool>;
#[doc = "Field `sw_rst_cmd` writer - "]
pub type SW_RST_CMD_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_TIMEOUT_CTRL_SW_RESET_SPEC, bool, O>;
#[doc = "Field `sw_rst_dat` reader - "]
pub type SW_RST_DAT_R = crate::BitReader<bool>;
#[doc = "Field `sw_rst_dat` writer - "]
pub type SW_RST_DAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_TIMEOUT_CTRL_SW_RESET_SPEC, bool, O>;
#[doc = "Field `reserved_15_11` reader - "]
pub type RESERVED_15_11_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn timeout_value(&self) -> TIMEOUT_VALUE_R {
        TIMEOUT_VALUE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reserved_7_4(&self) -> RESERVED_7_4_R {
        RESERVED_7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SW_RST_CMD_R {
        SW_RST_CMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SW_RST_DAT_R {
        SW_RST_DAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn reserved_15_11(&self) -> RESERVED_15_11_R {
        RESERVED_15_11_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_value(&mut self) -> TIMEOUT_VALUE_W<0> {
        TIMEOUT_VALUE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W<8> {
        SW_RST_ALL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd(&mut self) -> SW_RST_CMD_W<9> {
        SW_RST_CMD_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat(&mut self) -> SW_RST_DAT_W<10> {
        SW_RST_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Control/Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_timeout_ctrl_sw_reset](index.html) module"]
pub struct SD_TIMEOUT_CTRL_SW_RESET_SPEC;
impl crate::RegisterSpec for SD_TIMEOUT_CTRL_SW_RESET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_timeout_ctrl_sw_reset::R](R) reader structure"]
impl crate::Readable for SD_TIMEOUT_CTRL_SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_timeout_ctrl_sw_reset::W](W) writer structure"]
impl crate::Writable for SD_TIMEOUT_CTRL_SW_RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_timeout_ctrl_sw_reset to value 0"]
impl crate::Resettable for SD_TIMEOUT_CTRL_SW_RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
